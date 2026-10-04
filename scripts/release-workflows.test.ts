import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";

interface WorkflowShape {
  on?: unknown;
  permissions?: { contents?: unknown };
  jobs?: Record<
    string,
    {
      outputs?: Record<string, unknown>;
      steps?: Array<{ name?: string; if?: string; env?: Record<string, unknown> }>;
    }
  >;
}

function load(name: string): WorkflowShape {
  const parsed: unknown = Bun.YAML.parse(
    readFileSync(new URL(`../.github/workflows/${name}.yml`, import.meta.url), "utf8"),
  );
  if (typeof parsed !== "object" || parsed === null) {
    return {};
  }
  return parsed as WorkflowShape;
}

function findStep(
  jobs: WorkflowShape["jobs"],
  job: string,
  name: string,
): { if?: string; env?: Record<string, unknown> } {
  const step = jobs?.[job]?.steps?.find((step) => step.name === name);
  if (!step) {
    throw new Error(`step ${job}/${name} not found`);
  }
  return step;
}

describe("fork publishing workflow boundaries", () => {
  test("packaging publishes on v* tags and main, with write permissions", () => {
    const packages = load("packages");
    expect(packages.on).toEqual({
      push: {
        branches: ["main"],
        tags: ["v*"],
        "paths-ignore": ["README.md", "RELEASE_VERSION"],
      },
      workflow_dispatch: null,
    });
    expect(packages.permissions?.contents).toBe("write");
  });

  test("version bump drives the package build and main release", () => {
    const jobs = load("packages").jobs;
    expect(jobs?.build?.outputs).toEqual({
      version: "${{ steps.version.outputs.value }}",
      bumped: "${{ steps.version.outputs.bumped }}",
    });
    const build = findStep(jobs, "build", "Build artifacts");
    expect(build.if).toContain("bumped == 'true'");
    const publish = findStep(jobs, "build", "Publish main-branch release");
    expect(publish.if).toContain("bumped == 'true'");
    const pin = findStep(jobs, "sync-readme", "Pin README and push");
    expect(pin.env?.VERSION).toBe("${{ needs.build.outputs.version }}");
    const arch = findStep(jobs, "arch-package", "Attach package to release");
    expect(arch).toBeTruthy();
    expect(jobs?.["arch-package"]?.["if"]).toContain("bumped == 'true'");
  });
});
