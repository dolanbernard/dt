# dt

dt stands for Δt, or Delta Time. dt is a CLI utility for doing quick DateTime calculations. Features include displaying current DateTime for UTC, local, or a specified timezone, adding/subtracting a duration from a DateTime, calculating time since/until between two DateTimes with mixed formats+timezones, and summing durations with varying formats. Most commands accept input DateTimes from any format and allow a user-specified format specifier for the output.

## Commands

| Command | Purpose                                                                             | Aliases         | Arguments                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
|---------|-------------------------------------------------------------------------------------|-----------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| now     | Prints current DateTime with specified timezone in specified format                 | n               | -l/--local  (optional) use local timezone (has precedence over -z)<br> -z/--tz     (optional) use a specific timezone<br> -f/--format (optional) specify output format                                                                                                                                                                                                                                                                                                                                         |
| add     | Adds a duration in various formats to current DateTime and prints the result        | a               | -l/--local  (optional) use local timezone (has precedence over -z)<br> -z/--tz     (optional) use a specific timezone<br> -f/--format (optional) specify output format<br> (delta)     (required) the duration to add                                                                                                                                                                                                                                                                                          |
| sub     | Subtracts a duration in various formats from current DateTime and prints the result | s               | -l/--local  (optional) use local timezone (has precedence over -z)<br> -z/--tz     (optional) use a specific timezone<br> -f/--format (optional) specify output format<br> (delta)     (required) the duration to subtract                                                                                                                                                                                                                                                                                     |
| since   | Calculates and prints the time since a DateTime in the past                         | snc             | -l/--ref-local-tz (optional) use local timezone for the past DateTime (has precedence over -z)<br> -z/--ref-tz       (optional) use specified timezone for the past DateTime<br> -c/--end-local-tz (optional) use local timezone for the future DateTime (has precedence over -t)<br> -t/--end-tz       (optional) use specified timezone for the future DateTime<br> -e/--end          (optional) the future DateTime (if omitted, uses current DateTime)<br> (start)           (required) the past DateTime  |
| until   | Calculates and prints the tine until a DateTime in the future                       | utl             | -l/--ref-local-tz (optional) use local timezone for the past DateTime (has precedence over -z) <br> -z/--ref-tz       (optional) use specified timezone for the past DateTime<br> -c/--end-local-tz (optional) use local timezone for the future DateTime (has precedence over -t)<br> -t/--end-tz       (optional) use specified timezone for the future DateTime<br> (start)           (optional) the past DateTime<br> -e/--end          (required) the future DateTime (if omitted, uses current DateTime) |
| sum     | Sums any number of durations in various formats and prints the total duration       | total           | -f/--format (optional) the output format specifier string (currently unused)<br> (durations) (required) a list of durations in mixed formats to be summed                                                                                                                                                                                                                                                                                                                                                      |
| timer   | Starts a timer for the sum of the specified durations                               | tmr, sleep, slp | (durations) (required) a list of durations in mixed formats to be summed                                                                                                                                                                                                                                                                                                                                                                                                                                       |

## Formats

### DateTimes

DateTime format specifiers at this point just follow the Rust chrono crate. A list of DateTime formatters can be found [here](https://docs.rs/chrono/latest/chrono/format/strftime/index.html). Additional support for other standards/languages is planned but not implemented at this time.

### Durations

Formats for durations/time deltas can be expressed in multiple ways.

#### Timestamp Representation

This representation looks similar to human-readable timestamps in most applications. A single number such as `30` will be interpreted as 30 seconds. Adding numbers the to the left (delimited by colons) will increse the tier of that time representation by one. So the result is that timestamps can be submitted of the form `days:hours:minutes:seconds`. Seconds is the base unit. To add quantities of larger units, location must be maintained. For example, one hour and one second must be `1:0:1`. Milliseconds may also be represented in this format, however, seconds must be specified as well, even if the second quantity is 0. A timestamp of 200 milliseconds therefore looks like `0;200`. Note that the millisecond portion is separated by a semicolon instead of a colon like the other time denominations.

#### Natural Duration Representation

Durations may also be represented with explicit, human-readable time units. While durations are always optimally expressed in outputs by `dt`, duration inputs are very flexible. For example, `1.5h` and `90m` are both interpreted exactly the same. You can view a list of currently implemented [here](src/util/duration.rs#L21).

## Examples

The following sections show some usage examples. Each command example uses the proper, full name of the command. For a list of shortened command aliases, see [Commands](#Commands)

### Now

#### Get the current time in UTC

`dt now`

#### Get current local time

`dt now -l`

#### Get current time for Eastern time USA and Canada

`dt now "America/New York"` or `dt now America/New_York` or `dt now -z EST` or `dt now -z EDT`

(Note that EST and EDT will both always return the same time. Daylight savings time is automatically calculated based on the date.)

#### Get current local DateTime with specific format

`dt now -l -f "%Y-%m-%d %H:%M:%S"`

(Example output: 2004-06-04 15:00)

### Add

#### Calculate DateTime in local time 17 days from current DateTime

`dt add -l 17d`

#### Calculate DateTime in UTC 90 minutes from current DateTime

`dt add 90m` or `dt add 1.5h`

#### Calculate DateTime in Japan 12 hours, 34 minutes, 27 seconds, and 320 milliseconds from now

`dt add -z JST '12:34:27;320'`

### Sub

This command is exactly the same as [Add](#Add) except it subtracts instead of adding

### Since

#### Calculate my age 

`dt since -z EST "1995-10-12 02:00"`

#### Calculate time between a recent point in your local timezone and a distant time in a different timezone

`dt since -z "Africa/Abidjan" "1987-05-03 23:46" -c -e 2020-01-01`

(output: 32 years 8 months 1 week 3 days 8 hours 14 minutes)

### Until

This is basically the opposite of [Since](#Since), but the required argument is swapped

#### Calculate time until the year 3000

`dt until 3000-01-01`

#### Calculate time between WWI and WWII

`dt until -r 1918-11-11 1939-09-01`

(output: "20 years 9 months 4 weeks 1 day")

### Sum

#### Add up a bunch of durations

`dt sum 30m 1.25h 3s 45y 1.5w 2M 2ms`

(output: "45 years 2 months 1 week 3 days 13 hours 45 minutes 3 seconds 2 milliseconds")

### Timer

#### Sleep for 2 hours

`dt timer 1.5h 30m`
