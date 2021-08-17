# lprefix
A small tool to prefix lines in a script.

---

While using `xargs`, I found it hard to use `sed` to prepend something to lines in the output because of its regex interpretation.
That's why I hacked this together.

## Install

```shell
cargo install lprefix
```

## Example

Example of how I use it:
```shell
forall () {
	local P=${P:-1} 
	local ARG="cd {} ; pwd ; $@" 
	find . -type d -depth 1 -maxdepth 1 | xargs -n 1 -P ${P} -I {} bash -c "$ARG | lprefix '{}'"
}
```