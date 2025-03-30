# Dotfile-Manager

This is a learning project to bring myself to learn some Rust.

The plan is to parse a yaml/json/toml file and configure either a mac or linux system.

An example config should look somewhat like this:

```yaml
nvim: git@github.com:Sironheart/neovim-config.git
projectsConfig:
	basePath: ~/projects
	useGitSourceParts: true

files:
	- basePath: ~/.rgignore
    content: |
      foo bla bar miep baz
	- basePath: ~/.ssh/config
    content: |
      Host *
        Set Term foo
        IdentityAgent ~/.1password/agent.sock

macos:
	brew:
		package:
			- pinentry-mac
		casks:
			- 1password
			- discord
			- spotify
	configOptions:
		dock:
			autohide: true
			showRecents: true
		finder:
			showAllFiles: true
		global:
			InterfaceStyle: "Dark"
			automaticCapitalize: false

linux:
	apt:
		- nginx
	dnf:
		- httpd
	configOptions:
		foo: bar
```
