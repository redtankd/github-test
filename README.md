[TOC]

---
# Personal Configuraiton

My personal configuration files is managed by Git.

## $HOME ##

* `.gitconfig`
* `.gitignore`
* `.config/fish/config.fish` Fish's profile
* `.config/fish/functions/` Fish's extension functions
* `.profile` ksh
* `.my.cnf` mysql
* `.npmrc` npm

## $HOME/etc ##

* `redis-6379.conf`

## $HOME/bin ##

* `gradlew` The recursive wrapper command for gradle. See https://gist.github.com/dougborg/4278116
* `vpn`

--- ---
# OS X #

## Homebrew ##

### install Homebrew ###

`ruby -e "$(curl -fsSL https://raw.github.com/Homebrew/homebrew/go/install)"`

Homebrew is installed at `/usr/local`, which does not exist in OS X.

Homebrew packages's $prefix is `/usr/local/`. The packages is installed at `/usr/local/Cellar` and symlinked to `/usr/local`. 

### update Homebrew ###

`brew update`

### install, upgrade and see packages ###

```
brew install pcre

brew install node
brew info node
brew upgrade node
```

Before upgrading a package, see the package's information with `brew info`

## Fish ##

Append `/usr/local/bin/fish` to `/etc/shells`.

```
brew install fish
chsh /usr/local/bin/fish
```

* `~/.config/fish/config.fish` fish's profile
* `~/.config/fish/functions` user's functions
* `~/.config/fish/functions/fish_prompt.fish` my favorite prompt
* `~/.config/fish/functions/fish_user_key_bindings.fish` my key bindings

## Docker ##

### Install Oracle's VirtualBox ###

Change the default path for Virtual Machine to `/Users/redtank/Documents/VirtualBox VMs`

### Install Docker ###

Install docker and docker-machine with Homebrew, see [https://docs.docker.com/installation/mac/]

### Useful commands ###

* switch environment variables for docker machines
`eval (docker-machine env local)`

* execute a command in a running container
`docker exec -it gitlab bash`

* execute a command using one container's volumes
`docker run -it --rm --volumes-from=mysql sameersbn/mysql:latest mysql -uroot`  
---
# MySql #

```
mysql.server start
```

---
# GitLab #

## Installation with Docker ##

see [https://github.com/sameersbn/docker-gitlab]

### Install MySql ###

`docker pull sameersbn/mysql:latest`

```
docker run --name=mysql-gitlab -d \
-e 'DB_NAME=gitlabhq_production' -e 'DB_USER=gitlab' -e 'DB_PASS=password' \
-v ~/var/docker/gitlab/mysql/:/var/lib/mysql  \
-v ~/etc/docker/entrypoint-mysql-osx.sh:/entrypoint.sh \
mysql:5.6.24
```

note: OS X shared volumes in VirtualBox is mounted with user id 1000, unable to change. So the docker app's entrypoint script is override. 

### Install Redis ###

`docker pull sameersbn/redis:latest`

`docker run --name=redis-gitlab -d sameersbn/redis:latest`

### Install Gitlab ###

`docker pull sameersbn/gitlab:7.9.4`

```
docker run --name='gitlab' -d \
--link mysql-gitlab:mysql --link redis-gitlab:redisio \
-p 10022:22 -p 10080:80 \
-e 'SMTP_HOST=' -e 'SMTP_USER=USER@gmail.com' -e 'SMTP_PASS=PASSWORD' \
-v ~/var/docker/gitlab/gitlab/:/home/git/data \
sameersbn/gitlab:7.9.4
```

See configuration parameters at [https://github.com/sameersbn/docker-gitlab#available-configuration-parameters]

the Gitlab's default user is

```
username: root
password: 5iveL!fe
```

git clone: `git clone ssh://git@192.168.59.103:10022/redtank/gitlab-test.git`. `ssh:` is required.

## Installation with bitnami's stack ##

for bitnami's Virtual Machine GitLab Stack

### start sshd ###

```
cd /etc/init
sudo cp ssh.conf.back ssh.conf
reboot
```

### smtp ###

```
cd /opt/bitnami/apps/gitlab/htdocs/config/environments/
sudo cp production.rb production.rb.sample
ls -l production.rb*
sudo vi production.rb
```

edit: `config.action_mailer.delivery_method = :smtp` replace `sendmail` with `smtp`

add:

```
  config.action_mailer.smtp_settings = {
    address: "smtp.126.com",
    port: 25,
    user_name: "redtank@126.com",
    password: "",
    domain: "smtp.126.com",
    authentication: :plain,
    enable_starttls_auto: true
  }
```

```
cd /opt/bitnami/apps/gitlab/htdocs/config/
sudo cp gitlab.yml gitlab.yml.sample
ls -l gitlab.yml*
sudo vi gitlab.yml
```

edit:  `email_from: redtank@126.com`


```
sudo /opt/bitnami/ctlscript.sh restart gitlab_sidekiq
sudo /opt/bitnami/ctlscript.sh restart apache
```

### sendmail ###

not finished

```
sudo apt-get update
sudo apt-get install sendmail
```

---
# Redmine #

Migrating FMS' Redmine to Docker. The Data Store is `~/var/docker/redmine`.

## Installation with Docker ##

see [https://github.com/sameersbn/docker-redmine]

### Install MySql ###

`docker pull mysql:5.6.24`

```
docker run --name=mysql-redmine -d \
-e 'DB_NAME=redmine' -e 'DB_USER=redmine' -e 'DB_PASS=redmine' \
-v ~/var/docker/redmine/mysql/:/var/lib/mysql  \
-v ~/etc/docker/entrypoint-mysql-osx.sh:/entrypoint.sh \
mysql:5.6.24
```

note: Only root can write to OSX shared volumes in VirtualBox, so the docker app needs a start script. 

Fix the user redmine's privileges:
```
docker exec -it mysql-redmine mysql

use mysql;
update user set host='%' where user='redmine';
update db set host='%' where user='redmine';
FLUSH PRIVILEGES;
```

### Install Redmine ###

Comment out the group xapian-full for the dmsf plugin  in `~/var/docker/redmine/redmine/plugins/redmine_dmsf/Gemfile`

`docker pull sameersbn/redmine:2.5.3-1`

```
docker run --name=redmine --link mysql-redmine:mysql -d \
-v ~/var/docker/redmine/redmine/:/home/redmine/data \
-p 11080:80 \
sameersbn/redmine:2.5.3-1
```

See configuration parameters at [https://github.com/sameersbn/docker-redmine#available-configuration-parameters]

## Old Redmine Upgrade 补充说明 ##

1、备份导入

mysql -u redmine --password=redmine redmine < redmine_14_01_24

2、bundler安装

gem install bundler --user-install

3、mysql2安装

export GEM_HOME=/Users/redtank/opt/redmine-2.4.2/vendor/bundle/ruby/2.0.0/
gem install mysql2 -- --with-mysql-config=/Users/redtank/opt/mysql/bin/mysql_config
profile增加: export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:~/opt/mysql/lib

4、安装redmine的bundle

/Users/redtank/.gem/ruby/2.0.0/bin/bundle install --path=/Users/redtank/opt/redmine2.4.2/vendor/ --without development test rmagick

5、安装dmsf

删除Gemfile中的xapian-full

---
# other #

## ssh tunnel ##

ssh -L 30022:192.168.2.205:22 dev@222.66.16.96 -p 9001

