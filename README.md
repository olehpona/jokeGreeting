#jokeGreeting
This is simple app that will add to your /etc/motd.d new joke on every login.
It also have some setup like params from joke api or auto installation or uninstallation
```
Usage: jokeGreeting [OPTIONS]

Options:
  -i, --install <INSTALL>          Install script to local profile, global profile or custom path [local_profile, global_profile, <path>]
  -u, --uninstall <UNINSTALL>      Remove script from local profile and global profile or from custom path [local_profile, global_profile, <path>]
  -g, --gen-script                 Generate run script
      --category <CATEGORY>        set joke category. Details https://sv443.net/jokeapi/v2/
  -l, --language <LANGUAGE>        set joke language. Details https://sv443.net/jokeapi/v2/
  -b, --black-list <BLACK_LIST>    set joke black_list flags. Details https://sv443.net/jokeapi/v2/
  -j, --joke-type <JOKE_TYPE>      set joke type. Details https://sv443.net/jokeapi/v2/
      --content <CONTENT>          set joke content. Details https://sv443.net/jokeapi/v2/
  -e, --export-path <EXPORT_PATH>  set export location [default: /etc/motd.d/jokes]
  -h, --help                       Print help
```
