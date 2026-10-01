import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";

interface WorkflowShape {
  on?: unknown;
  permissions?: { contents?: unknown };
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

describe("fork publishing workflow boundaries", () => {
  test("packaging publishes on v* tags and main, with write permissions", () => {
    const packages = load("packages");
    expect(packages.on).toEqual({
      push: { branches: ["main"], tags: ["v*"] },
      workflow_dispatch: null,
    });
    expect(packages.permissions?.contents).toBe("write");
  });

  test("normal PR CI remains enabled", () => {
    expect(load("ci").on).toMatchObject({ pull_request: expect.anything() });
  });
});
