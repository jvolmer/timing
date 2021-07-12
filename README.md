# Timing

This project should help ease the weekly submission of timesheets.

## The problem

On a workday, I am required to note the tickets I work on with start and end time to a specific website. Once a week I have to submit my entries.
Problematic is, that there is no automatic validation: I can add overlapping time entries, forget a mandatory description or even write down the wrong project. The result: Once a week I have to sit down and check all items manually - and once in a while I am still missing some errors.

## Solution

I really want to automate this time sheet validation. The idea of this crate (additional to learning Rust) is the following:

I write my timings in a simple text file.

I apply this command line tool to file and it tells me all errors I make.

After I fixed everything, I can send the timesheet data to the timesheet website via another command.

## Validations

Validations I have in mind so far

* Non-Overlapping time entries

* Correct project selections

* Correctly filled out description fields (including a ticket number)

* Breaks after specific working durations

## Further ideas

* Have an analysis of my work: how many hours in total, how many hours per project, ...

* Automatic fixes like adding breaks after appropriate durations but leave the total working time un
touched
