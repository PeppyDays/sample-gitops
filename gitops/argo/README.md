# Argo Deployment

## Setup Order

```bash
# bootstrap/phase-1
# deploy argo applications
kustomize build | kubectl apply -f -

# bootstrap/phase-2
# deploy argo self-managed applications
kustomize build | kubectl apply -f -

# bootstrap/phase-3
# deploy external packages as argo cd applications for global use, e.g. external secrets
kustomize build | kubectl apply -f -

# bootstrap/phase-5
# deploy internal applications which are defined in products and applications
# no need to do something manually after deploying this
kustomize build | kubectl apply -f -
```
aa
