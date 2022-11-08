# Reywen 
A general purpose bot running with an asynchronous 'plugin' system, built on the RevX2 library.
Written in funny crab language :crab:.

RevX2 can be used to construct your own bot!

# Example use

## RevX1

```rust

if content == "?hello" {

  rev_send(data.token.clone(), channel.clone(), "hello user".to_string());

}
```
## RevX2
```rust
 match &content as &str {

        "?hello" => send(data, message, "hello user".to_string()).await,
        _ => return
    };

```
Using `match` instead of `else if` is more compact, and allows for more complex functions


While the difference in syntax is minimal, there are many benefits of RevX2 over RevX1, such as
- async
- error handling
- serde_json (insteaed of ajson)
- modular rust crates as 'plugins'
Features specific to Messages
- masqurade (properly implemented)
- replies with mentioning
And new features comming soon!
- Mongodb support (nearly complete)
- Discord integration (help wanted)


# Getting started
```bash
git clone https://github.com/toastxc/Reywen-Revolt.git
cd Reywen-Revolt
nohup cargo build --release &
vim reywen.json
```


For further information of [Revolt](https://developers.revolt.chat) or [Reywen](https://github.com/toastxc/Reywen-Revolt/issues)
