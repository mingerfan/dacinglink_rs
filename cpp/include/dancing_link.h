#include <vector>
#include <memory>
#include "rust/cxx.h"

struct DLX
{
  int n, m, size;
  int U[5002], D[5002], L[5002], R[5002], Row[5002], Col[5002];
  int H[5002], S[5002];

  std::vector<int> ans_path;
  int ansd;

  void init(int _n, int _m);
  void Link(int r, int c);
  void remove(int c);
  void resume(int c);
  int f() const;
  void dance(int dep, std::vector<int> &path);
  const std::vector<int>& get_res() const;
};

std::unique_ptr<DLX> new_DLX();