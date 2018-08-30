# Youdao_rs

A command line tool for You Dao dictionary written in Rust

## Installation

#### Homebrew

```bash
brew tap hotogwc/tap
brew install hotogwc/tap/YoudaoRS
```

## Speech

___NOTICE:___ Currently, speech feature is only available for MacOS/Linux.

#### Mac OS

```bash
brew install mpg123
```

#### Ubuntu

```bash
sudo apt-get install mpg123
```

#### CentOS

```bash
yum install -y mpg123
```

## Usage

1. Query

```text
youdao_rs <word(s) to query>
```

1. Query with speetch (__Available for MacOS & Linux__)

```text
youdao_rs <word(s) to query> -v
```

1. Query and show more example sentences

```text
youdao_rs <word(s) to query> -m
```

##  