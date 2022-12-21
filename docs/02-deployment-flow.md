# Deployment Flow

The deployment of a repo goes as follows:

1. Commits are sent to the webhook
2. If the ref doesn't match ``repositoty.ref``, a response is returned
3. A commit status with a state of pending is created
4. If ``deployment.pull`` is set, that command is run; if that isn't the case, ``git pull`` is run
5. If the pull step fails, a commit status with a state of failed and a description of
``deployment.descriptions.failed_pull`` is created
6. Each command on the ``commands`` table is run; if it fails, a commits status with a state of failed and a description
of ``(deployment.descriptions.failed_build).replace("{step}", deployment.descriptions.display)``
7. A commit status with a state of success is created with a description of ``deployment.descriptions.success``

If an unknown error is returned anywhere, a commit status with a state of failed and a description of 
``deployment.descriptions.unknown_error`` is created