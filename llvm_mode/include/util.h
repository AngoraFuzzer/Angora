#ifndef UTIL_H
#define UTIL_H

#define GET_OR_INSERT_READONLY_FUNCTION(callee_obj, ret_ty, func_name, ...)    \
  {                                                                            \
    FunctionType *callee_obj##Ty =                                             \
        FunctionType::get(ret_ty, __VA_ARGS__, false);                         \
    AttributeList AL;                                                          \
    AL = AL.addAttribute(M.getContext(), AttributeList::FunctionIndex,         \
                         Attribute::NoUnwind);                                 \
    AL = AL.addAttribute(M.getContext(), AttributeList::FunctionIndex,         \
                         Attribute::ReadOnly);                                 \
    callee_obj = M.getOrInsertFunction(func_name, callee_obj##Ty, AL);         \
  }

#define GET_OR_INSERT_FUNCTION(callee_obj, ret_ty, func_name, ...)             \
  {                                                                            \
    FunctionType *callee_obj##Ty =                                             \
        FunctionType::get(ret_ty, __VA_ARGS__, false);                         \
    AttributeList AL;                                                          \
    AL = AL.addAttribute(M.getContext(), AttributeList::FunctionIndex,         \
                         Attribute::NoUnwind);                                 \
    callee_obj = M.getOrInsertFunction(func_name, callee_obj##Ty, AL);         \
  }

#endif