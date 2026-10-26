"""Exercise the release checker through its command-line boundary."""
import subprocess
import sys
import tempfile
from pathlib import Path
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / "check-release.py"


class ReleaseCheck(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / "src").mkdir()
        (self.root / "src/lib.rs").write_text("")
        (self.root / "Cargo.toml").write_text('[package]\nname="release-fixture"\nversion="0.1.0"\nedition="2021"\n')
        (self.root / "CHANGELOG.md").write_text("# Changelog\n\n## [Unreleased]\n\n- Next change.\n\n## [0.1.0]\n\n- Initial library.\n")

    def run_check(self, *args):
        return subprocess.run([sys.executable, str(SCRIPT), "--root", str(self.root), *args], capture_output=True, text=True)

    def test_dry_run_reports_package_without_releasing(self):
        result = self.run_check("--dry-run")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout, "package=release-fixture version=0.1.0\nrelease-check: dry-run passed; release entry and signed tag still required\n")

    def test_matching_tag_and_notes_pass(self):
        result = self.run_check("--tag", "v0.1.0")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout, "package=release-fixture version=0.1.0\nrelease-check: tag metadata ready\n")

    def test_mismatched_tag_fails(self):
        result = self.run_check("--tag", "v0.2.0")
        self.assertEqual(result.returncode, 1)
        self.assertIn("does not match workspace version v0.1.0", result.stderr)

    def test_missing_release_notes_fail(self):
        (self.root / "CHANGELOG.md").write_text("## [Unreleased]\n\n- Next change.\n")
        result = self.run_check("--tag", "v0.1.0")
        self.assertEqual(result.returncode, 1)
        self.assertEqual(result.stderr, "release-check: CHANGELOG.md needs a nonempty [0.1.0] section\n")

    def test_mixed_workspace_versions_fail(self):
        with (self.root / "Cargo.toml").open("a") as file:
            file.write('\n[workspace]\nmembers=["member"]\n')
        member = self.root / "member"
        (member / "src").mkdir(parents=True)
        (member / "src/lib.rs").write_text("")
        (member / "Cargo.toml").write_text('[package]\nname="member-fixture"\nversion="0.2.0"\nedition="2021"\n')
        result = self.run_check("--dry-run")
        self.assertEqual(result.returncode, 1)
        self.assertEqual(result.stderr, "release-check: workspace package versions differ\n")


if __name__ == "__main__":
    unittest.main()
