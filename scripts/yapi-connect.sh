#!/usr/bin/env bash

set -u
set -e
set -x

namespace=reddwarf-toolbox
kubectl config use-context context-reddwarf
POD=$(kubectl get pod -l app.kubernetes.io/instance=yapi-1639757860 -n ${namespace} -o jsonpath="{.items[0].metadata.name}")
kubectl port-forward ${POD} 3000:3000 -n ${namespace} --address='0.0.0.0'



