#include "clib.h"
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

struct NLOpaqueType {
  NLOpaqueTypeData value;
  void *target;
  NLOpaqueTypeAction action;
};

NLOpaqueType *NLOpaqueTypeCreate(NLOpaqueTypeData value) {
  NLOpaqueType *instance = (NLOpaqueType *)malloc(sizeof(NLOpaqueType));
  instance->value = value;
  return instance;
}

void NLOpaqueTypeDelete(NLOpaqueType *instance) { free(instance); }

NLOpaqueTypeData NLOpaqueTypeGetValue(NLOpaqueType const *instance) {
  return instance->value;
}

void NLOpaqueTypeSetValueToSumOfValues(NLOpaqueType *instance, int32_t count,
                                       ...) {
  va_list ap;
  va_start(ap, count);

  double sum = 0;
  for (int i = 0; i < count; ++i) {
    sum += va_arg(ap, int32_t);
  }
  va_end(ap);
  instance->value.integer = sum;
}

void NLOpaqueTypeRegisterCallback(NLOpaqueType *instance, void *target,
                                  NLOpaqueTypeAction action) {
  instance->target = target;
  instance->action = action;
}

void NLOpaqueTypeTriggerCallback(NLOpaqueType const *instance) {
  if (instance->target && instance->action) {
    instance->action(instance->target, instance->value.integer);
  }
}

void NLInitVectorIncrementer(int64_t *p, int count) {
  for (int i = 0; i < count; ++i) {
    p[i] = i;
  }
}