#!/bin/bash

command=$1; shift
array=("$@")

case $command in
    "install") 
        if [[ ${#array[@]} -eq 0 ]]; then
            PYTHONPATH=src python -c 'import version_checker; version_checker.update_model_versions()'
        else
            PYTHONPATH=src python -c 'import sys, version_checker; version_checker.update_model_versions(sys.argv[1:])' "${array[@]}"
        fi
    ;;
    "remote")
        PYTHONPATH=src python -c 'import version_checker; version_checker.get_status()' 
    ;;
    "upload")
        if [[ ${#array[@]} -eq 0 ]]; then
            PYTHONPATH=src python -c 'import sys, version_checker; version_checker.upload_manager()' 
        else
            PYTHONPATH=src python -c 'import sys, version_checker; version_checker.upload_manager(sys.argv[1:])' "${array[@]}"
        fi
    ;;
    "status") 
        PYTHONPATH=src python -c 'import version_checker; version_checker.get_status(True)'
    ;;
    "clean") rm -r models ;;
    *) echo "Usage: $0 {install|clean|status|remote}" ;;
esac
