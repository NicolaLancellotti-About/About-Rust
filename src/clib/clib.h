#ifndef file_h
#define file_h

#include <stdio.h>
#include <stdbool.h>

typedef struct NLOpaqueType NLOpaqueType;

typedef struct
{
    int32_t integer;
    bool boolean;
} NLOpaqueTypeData;

typedef void (*NLOpaqueTypeAction)(void *, int32_t);

NLOpaqueType *NLOpaqueTypeCreate(NLOpaqueTypeData value);

void NLOpaqueTypeDelete(NLOpaqueType *instance);

NLOpaqueTypeData NLOpaqueTypeGetValue(NLOpaqueType const *instance);

void NLOpaqueTypeSetValueToSumOfValues(NLOpaqueType *instance,
                                       int32_t count, ...);

void NLOpaqueTypeRegisterCallback(NLOpaqueType *instance,
                                  void *target,
                                  NLOpaqueTypeAction action);

void NLOpaqueTypeTriggerCallback(NLOpaqueType const *instance);

void NLInitVectorIncrementer(int64_t *p, int count);

#endif
