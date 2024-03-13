
# DSLabs Log Visualization Tool
For students using DSLabs in their class projects, reading through logs while debugging can be a challenge. It is
valuable to log more events in your system, but as a tradeoff, the more logs you produce the harder it is to actually
gain any insight from those logs. Enter this tool! It is designed to making reading logs easier.

## How it works
This tools works by taking an input file full of structured logs, parsing it, and then displaying it in a TUI. With this
tool, you can:
- Easily page and navigate the list of events
- Search for events with a simple declarative query language

## Getting Starting
### Prerequisites
To use this tool, you first have to integrate a compatible logger into your DSLabs project. 
TODO

### Installing
TODO

### Usage

```shell
$ ./run-tests <whatever params> 2> <path to logs file>
$ log-viz <path to logs file>
```
