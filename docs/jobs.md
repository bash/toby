# Jobs

A job is triggered using the [`/v1/jobs/:project`](./api.md) endpoint. Each job receives a unique id (incremental).

## Execution Order

Jobs are executed in the same order that they were queued. Note however that this is only true for jobs of the same project.  
This will allow for future changes to run jobs for different projects in parallel.

## Working Directory

Toby runs each job in a blank directory that is erased after the job has completed.

## Environment

Jobs inherit environment variables from the `tobyd` process.
Additional variables can be set using the [`environment` section](./config.md#the-environment-section-optional).

### Special Environment variables

These environment variables take precedence over the variables set in the `[environment]` section.

| **name**           | **description**                             |
| ------------------ | ------------------------------------------- |
| `TOBY_JOB_ID`      | The current job id.                         |
| `TOBY_JOB_TRIGGER` | The job's trigger (`webhook` or `telegram`) |


## Logs

Toby stores logs in `/var/log/toby/jobs`. Log files have the format `<project>-<id>.log`.