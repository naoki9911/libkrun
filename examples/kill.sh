#!/bin/bash

kill $(ps aux | grep chroot_vm | awk '{print $2}')

