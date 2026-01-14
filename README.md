# Simpan Kan Filemu
Simple filehosting.
I think there are many software that offer filehosting solutions. Let's reinvent the wheels, lol.

## Install
first you need to create file .env from env_sample by
```shell
mv env_sample .env
```
and adjust with your setup then you need to create db with sqlx and run migration 
```shell
sqlx db create && sqlx migrate run
```
and enjoy
## Usage
Just run with cargo
```shell
cargo run
```
then you can access from curl by using one of 4 options :
- /manage/upload/ - need to add file in curl request
- /manage/list/
- /manage/rename/ - need to add file and id, you can get id from list
- /manage/delete/ - only need to fill id
the default username and password are admin/admin
## License
[MIT](https://choosealicense.com/licenses/mit/)
