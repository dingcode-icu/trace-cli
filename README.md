# tracer-cli
:raised_eyebrow:JUST MORE SIMPLE THEN WEB -  trace command line tool to view the traces;


### Desc 
rust 1.6+

main dependents:
* clap
* ureq
* prettytable-rs

### Build 
JUST 

>cargo build --release --target=$target 

### Run 
linux&&Mac:
> chmod a+x $path/tracer_cli
> cd $path 
> ./tracer_cli -h


windows:
>./tracer_cli.exe -h

### Usage 
The usual way of using and also most important subcommand in tracer-cli

#### **board**

>show all bug trace base info 

```shell
./tracer-cli board                                     #show all base info
./tracer-cli board -t ${engine|innerjs|bundlejs|unknown}  #show typeed bug trace info 
```

#### **login**

>record the bug fixer usr info in github

```shell
./tracer-cli login                                     #it will jump to web in github to login
```

#### **fix**

>set the state of trace like "i have fix it!"

```shell
./tracer-cli fix "uncauch error:asdfa131/index.js"     #then fixed will +=1, use board command to check it!
```
