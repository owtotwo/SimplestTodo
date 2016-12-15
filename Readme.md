# TODO #
Manage Todo list as simple as possible on command line.

## Usage (Linux) ##
### Basic Usage ###
1. Build the program (Install [Rust](https://www.rust-lang.org/) stable version at first)  
    Install Rust (you can try "sudo" if it can not run):  
    `$ curl -sSf https://static.rust-lang.org/rustup.sh | sh`  
    Clone reposity:  
    `$ git clone https://github.com/owtotwo/SimplestTodo.git`  
    Change working directory:  
    `$ cd SimplestTodo`  
    Build it:  
    `$ cargo build --release`  

2. Copy the program to the path of your local executable binary file such as `/usr/local/bin`   
    `$ sudo cp ./target/release/todo /usr/local/bin`  

3. Set environment variable `TODO_PATH` for the path of the storage file such as `~/.todo`  
    `$ echo "export TODO_PATH=$HOME/.todo" >> ~/.profile && source ~/.profile`  

4. Add some notes  
    `$ todo 我唔知你系度讲紧D乜野`  
    `$ todo "Just One Last Dance"` 

5. List all notes  
    `$ todo`  
    ``` Output
    [ 1] Just One Last Dance 
    [ 2] 我唔知你系度讲紧D乜野 
    ```

6. Delete a note  
    `$ todo 1`  
    `$ todo`    
    ``` Output
    [ 1] 我唔知你系度讲紧D乜野 
    ```

### Advanced Usage ###
1. Set your username in Git, check it by:  
    `$ git config user.name`  
   and set it if output nothing:  
    `$ git config --global user.name "<your_user_name>"`

2. Set environment variable `GITHUB_ACCESS_TOKEN` by [Creating an access][1] for the sync 
   of your todolist such as `2a87e7cac314e2a634961e760c31bec902733c9b`.  
   (Ps: On step five on link _Select the scopes_ you are required to select the `gist` 
    checkbox.)  
    `$ echo "export GITHUB_ACCESS_TOKEN=<your_personal_access_token_here>" >> 
     ~/.profile && source ~/.profile`

3. Synchronization (through gist)  
   (Ps: This action will merge the todo items between local and non-local. It will create a gist 
 if you have no gist to store the todo list file.)  
    `$ todo sync`  
    ``` Output
    Sync...
    Waitting for gist download...
    Waitting for gist upload...
    Success to upload gist
    Done!
    ```
  [1]: https://help.github.com/articles/creating-an-access-token-for-command-line-use/

4. Upload (to gist)  
   (Ps: Pay attention that this action will overwrite the gist instead of merge)  
    `$ todo upload`  
    ``` Output
    Waitting for gist upload...
    Success to upload gist
    ```

5. Sync by an existed gist  
   (Ps: You can share the same todolist in different platform by gist)  
    `$ cat $TODO_PATH`  
    ``` Output
    {
        "todolist": [
            ...
        ],
        "gist_id": "12340e28693036cc000a48f59b75c868"
    }
    ```  
   Or  
    ``` Output
    {
        "todolist": [
            ...
        ],
        "gist_id": null
    }
    ```
   You can change the "gist_id" to the id of your existed gist which includes file 
   `.todo`. After your modification:  
    `$ cat $TODO_PATH`  
    ``` Output
    {
        "todolist": [
            ...
        ],
        "gist_id": "<the id of your existed gist>"
    }
    ```
   And then you just sync again:  
    `$ todo sync`  


Everything will be Ok.  


## Dependencies ##
* Rust 1.13+
* crate json = "0.11.2"


## License ##
* [LGPL](LICENSE)