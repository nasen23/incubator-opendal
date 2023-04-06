/**
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */


#ifndef _OPENDAL_H
#define _OPENDAL_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

/*
 Hello, OpenDAL!
 `opendal_operator` is the entry for all public blocking apis.
 */
typedef struct od_operator od_operator;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/*
 Create a new blocking `opendal_operator` with the given `scheme` and options.
 */
int od_operator_new(struct od_operator **operator_,
                    const char *scheme,
                    const char *const *const *options,
                    intptr_t options_len);

/*
 Free a previously created operator.
 */
void od_operator_free(struct od_operator *operator_);

int od_operator_read(struct od_operator *operator_,
                     const char *path,
                     uint8_t **buf,
                     uintptr_t *size);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _OPENDAL_H */
