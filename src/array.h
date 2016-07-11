#ifndef ARRAY_H
#define ARRAY_H

#include "arrow/array.h"

struct ArrayBox {
  std::shared_ptr<Array> sp;
  Array* array;
};

extern "C" {

}

#endif