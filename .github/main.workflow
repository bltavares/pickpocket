workflow "on push" {
  on = "push"
  resolves = ["clippy", "rustfmt"]
}

action "clippy" {
  needs = ["rustfmt"]
  uses = "bltavares/actions/clippy@rust-actions"
  args = ["autofix"]
  secrets = ["GITHUB_TOKEN"]
}

action "rustfmt" {
  uses = "bltavares/actions/rustfmt@rust-actions"
  args = ["autofix"]
  secrets = ["GITHUB_TOKEN"]
}
