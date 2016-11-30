# TODO #
Manage Todo list as simple as possible.

## Usage (Linux) ##
1. Build the program  
    `$> cargo build --release`  

2. Copy the program to the path of your local executable binary file such as `/usr/local/bin`   
    `$> sudo cp ./target/release/todo /usr/local/bin`  

3. Set environment variable `TODO_PATH` for the path of the storage file such as `~/.todo`  
    `$> echo "export TODO_PATH=$HOME/.todo" >> ~/.profile`  

4. Add some notes  
    `$> todo 我唔知你系度讲紧D乜野`  
    `$> todo "Just One Last Dance"` 

5. List all notes  
    `$> todo`  
``` Output
 [ 1] 我唔知你系度讲紧D乜野 
 [ 2] Just One Last Dance 
```

6. Delete a note
    `$> todo 1`  
    `$> todo`    
``` Output
 [ 1] Just One Last Dance  
```

## 