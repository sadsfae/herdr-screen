import unittest

from scripts.next_version import bump_level, next_version, tag_version


class BumpLevelTests(unittest.TestCase):
    def test_fix_bumps_patch(self):
        self.assertEqual(bump_level("fix: reload local config"), 1)

    def test_perf_bumps_patch(self):
        self.assertEqual(bump_level("perf: batch pane updates"), 1)

    def test_feat_bumps_minor(self):
        self.assertEqual(bump_level("feat: add logs workspace"), 2)
        self.assertEqual(bump_level("feat(scope): add logs workspace"), 2)

    def test_breaking_bumps_major(self):
        self.assertEqual(bump_level("feat!: drop legacy protocol"), 3)
        self.assertEqual(bump_level("fix(update)!: require digest"), 3)

    def test_unknown_kind_with_bang_does_not_bump(self):
        self.assertEqual(bump_level("foo!: drop api"), 0)

    def test_other_types_do_not_bump(self):
        for subject in [
            "docs: pin install commands",
            "chore: bump RELEASE_VERSION",
            "ci: cache zig",
            "test: cover prefix passthrough",
            "refactor: simplify resolve step",
            "release: 0.2.4",
        ]:
            self.assertEqual(bump_level(subject), 0)

    def test_non_conventional_subject_does_not_bump(self):
        self.assertEqual(bump_level("Merge pull request #22 from x"), 0)
        self.assertEqual(bump_level("typo in readme"), 0)


class NextVersionTests(unittest.TestCase):
    def test_patch(self):
        self.assertEqual(next_version("0.2.3", 1), "0.2.4")

    def test_minor(self):
        self.assertEqual(next_version("0.2.3", 2), "0.3.0")

    def test_major(self):
        self.assertEqual(next_version("0.2.3", 3), "1.0.0")

    def test_none_keeps_version(self):
        self.assertEqual(next_version("0.2.3", 0), "0.2.3")

    def test_rollover(self):
        self.assertEqual(next_version("1.9.9", 2), "1.10.0")
        self.assertEqual(next_version("0.19.19", 1), "0.19.20")

    def test_mixed_subjects_use_highest_bump(self):
        subjects = ["fix: reload config", "feat: add logs workspace"]
        level = max((bump_level(subject) for subject in subjects), default=0)
        self.assertEqual(next_version("0.2.3", level), "0.3.0")

    def test_invalid_base_raises(self):
        with self.assertRaises(ValueError):
            next_version("banana", 1)
        with self.assertRaises(ValueError):
            next_version("0.2.3-7802b16", 1)


class TagVersionTests(unittest.TestCase):
    def test_plain_tag(self):
        self.assertEqual(tag_version("v0.2.2"), "0.2.2")

    def test_per_push_tag(self):
        self.assertEqual(tag_version("v0.2.3-7802b16"), "0.2.3")
        self.assertEqual(
            tag_version("v0.2.3-7802b16fec005c3d1ace2f89a31456a1feacd56"), "0.2.3"
        )

    def test_rejects_other_shapes(self):
        for tag in ["v0.2", "0.2.3.4", "v0.2.3-abc", "v0.2.3-abc123"]:
            with self.assertRaises(ValueError):
                tag_version(tag)


if __name__ == "__main__":
    unittest.main()
