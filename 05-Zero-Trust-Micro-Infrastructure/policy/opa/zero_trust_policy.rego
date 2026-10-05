package madhatter.zerotrust

# Placeholder OPA/Rego policy for future enforcement.
# The current MVP validates controls with Python first.
# Later, this can be translated into executable OPA policy tests.

default allow = false

allow {
    input.service_identity != ""
    input.mtls_required == true
    input.auth_required == true
    input.network_policy_required == true
    input.least_privilege == true
    input.secrets_externalized == true
    input.observability_enabled == true
    input.public_exposure == false
}