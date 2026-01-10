# Simpan Kan Filemu
Simple filehosting.
I think there are many software that offer filehosting solutions. Let's reinvent the wheels, lol.

## Install
(TODO, because I haven't understand how to replicate setup on other machine, especially database)
## Usage
Just run with cargo
```shell
cargo run
```
then you can access from curl by using one of 4 options :
- /upload/ - need to add file in curl request
- /list/
- /rename/ - need to add file and id, you can get id from list
- /delete/ - only need to fill id
## License
[MIT](https://choosealicense.com/licenses/mit/)
