---
tags:
    - wip
    - python
    - svelte
---

[:simple-github: Repository](https://github.com/fcrozetta/ananke) |
<!-- [:material-download: Releases](https://github.com/fcrozetta/ananke/releases/latest) -->
# Ananke

## Introduction

!!! bug
    This Application is still in development and is not ready to be used

Ananke was created as a result of many years of struggling to keep track of changes in external libraries, sdk updates, cloud sdk releases with breaking changes, etc...

The idea of ananke is to keep track of all the tools and frameworks that a developer use. A pipeline broke cause azure changed their CLI? You will have the breaking change warning.

### How to run

**WIP** This is not working yet. Do not try to use it

A simple docker run has to be enough to have it working.
Does it have login? No. It is meant to run on your machine,as a helper application.

Maybe this could change in the future? :thinking:

This project separates the back and frontend, but it can always run together. To the user, both work as one.

## Frontend application :material-language-typescript:

This is where you will spend most of the time. this application starts witha dashboard that contains a calendar with the important events that happened. This could include not only sdk releases, but results from jobs as well.

The meat of this app is the job editor, where in a *blueprint* style you can automate your work. Then you set up this job to run on a scheduler.


## Backend application :material-language-python:

!!! danger
    You probably will never need to go into this part unless you want to improve, extend or understand the full application

Backend is a composition of 3 main things:
- API with DB connection
- Scheduler
- Job Engine (moirai)

Those 3 things work together to make the system work.
The API itself will control the regular tasks from the frontend: Settings, crud for tasks and jobs. It will also forward the actions to/from the scheduler like start, stop, add, remove jobs to the scheduler.

The scheduler is responsible for calling retreiving the tasks and passing it to engine. The results of the task should be logged as well.

Lastly the engine will process the job file, calling the programs and performing tasks as described in the jobfile.

The engine itself is called moirai, a python library created to process the tasks. You can see more of it [here](../Libraries/moirai.md).