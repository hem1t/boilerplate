<div align="center">

# Boilerplate


# How to use?

One thing to consider is that, every block will have a name and will be followed by it's arguments.

<div align="left">
    
### Symbols:


1. __$__ _(Directory Block)_

    > Any line starts with this symbol will be considered as block for directory and requires a directory name. Directory can have another directory block or file block.
   
        $name:"directory_name";
        !

2. __#__ _(File Block)_
   
   > This symbol takes a FileName as an argument and can have multiple FileContentBlock.

        #name:"file_name"; 
        !

3. __@__ _(File Content Block)_

    > This one doesn't requires any argument and needs to contain a parahgraph.
        
        @name;
        !

4. __!__ _(Ending of Block)_

    > Represents the ending of any block.

5. __:__ _(Argument starts)_

    > Should be at the start of the argument and also seperate them.

6. __;__ _(Argument ends)_ 

    > Represents the end of the arguments.

7. __'__ _(Parah)_

    > Anything between " (double quoutes) will be taken as it is.

8. __%__ _(List of tags)_ (Not yet Implemented)

    > Takes list of names of blocks in argument but first argument should be "include" or "exclude".
    >
    > If "include" is the first argument then it will only execute according to the list of names are given.
    >
    > And if "exclude" is the first argument then it will execute all other than the list of names.
    
        %name:"I/E":...:..;

    > Things to Note:
    >
    > 1. name of this list can be used while running the program.
    >
    > 2. 'I' stands for include and 'E' stands for Exclude. (you can also use lowercase).
    >
    > 3. This is not a block so, ! (Ending of Block) is not required.


## Example Code:

```python
    $projectfolder:'_P';
        $src:'src';
            #main:'main.rs';
                @inputm;
                    '
                    macro_rules! input {
                        ($t:tt) => {{
                            let mut temp = String::new();
                            std::io::stdin().read_line(&mut temp);
                            temp.split_whitespace().parse::<$t>().unwrap() 
                        }}
                    }
                    '
                @mainfn;
                    'fn main() {
                        unimplemented!();
                    }'
            !
        !
        
        #toml:'Cargo.toml';
            @package;
            '
            [package]
            name = "_P"
            version = "0.1.0"
            authors = ["_user _email"]
            edition = "2018"
            '
            @dependecies;
            '
            [dependecies]
            '
        !
    !
```
