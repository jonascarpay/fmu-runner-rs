#!/bin/bash
bindgen \
    --allowlist-type="(\w*fmi\w*)" \
    --allowlist-var="(\w*fmi\w*)" \
    --allowlist-function="(\w*fmi\w*)" \
    --disable-name-namespacing \
    --disable-nested-struct-naming \
    --translate-enum-integer-types \
    --wrap-unsafe-ops \
    --merge-extern-blocks \
    --rustified-enum="fmi2Status" \
    --rustified-enum="fmi2StatusKind" \
    --rustified-enum="fmi2Type" \
    fmi-standard/headers/fmi2Functions.h > src/fmi.rs