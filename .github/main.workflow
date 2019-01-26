workflow "on push" {
  on = "push"
  resolves = ["clippy", "rustfmt"]
}

action "clippy" {
  needs = ["rustfmt"]
  uses = "bltavares/actions/clippy@master"
  args = ["autofix"]
  secrets = ["GITHUB_TOKEN"]
}

action "rustfmt" {
  uses = "bltavares/actions/rustfmt@master"
  args = ["autofix"]
  secrets = ["GITHUB_TOKEN"]
}