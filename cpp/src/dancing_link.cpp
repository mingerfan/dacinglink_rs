#include "dancinglink/cpp/include/dancing_link.h"
#include <iostream>

using namespace std;
#define INF 0x3f3f3f3f

unique_ptr<DLX> new_DLX()
{
  return make_unique<DLX>();
}

void DLX::init(int _n, int _m)
{
  n = _n, m = _m;
  for (int i = 0; i <= m; i++)
  {
    S[i] = 0;
    U[i] = D[i] = i;
    L[i] = i - 1, R[i] = i + 1;
  }
  R[m] = 0, L[0] = m;
  size = m;
  for (int i = 1; i <= n; i++)
    H[i] = -1;
  ansd = INF;
}

void DLX::Link(int r, int c)
{
  ++S[Col[++size] = c];
  Row[size] = r;
  D[size] = D[c];
  U[D[c]] = size;
  U[size] = c;
  D[c] = size;
  if (H[r] < 0)
  {
    H[r] = L[size] = R[size] = size;
  }
  else
  {
    R[size] = R[H[r]];
    L[R[H[r]]] = size;
    L[size] = H[r];
    R[H[r]] = size;
  }
}

void DLX::remove(int c)
{
  for (int i = D[c]; i != c; i = D[i])
  {
    L[R[i]] = L[i];
    R[L[i]] = R[i];
  }
}

void DLX::resume(int c)
{
  for (int i = U[c]; i != c; i = U[i])
  {
    L[R[i]] = R[L[i]] = i;
  }
}

int DLX::f() const
{
  vector<int> v(size + 10);
  int ret = 0;
  for (int c = R[0]; c != 0; c = R[c])
    v[c] = true;
  for (int c = R[0]; c != 0; c = R[c])
    if (v[c])
    {
      ret++;
      v[c] = false;
      for (int i = D[c]; i != c; i = D[i])
        for (int j = R[i]; j != i; j = R[j])
          v[Col[j]] = false;
    }
  return ret;
}

void DLX::dance(int d, vector<int> &path)
{
  cout << "d: " << d << endl;
  if (d + f() > ansd)
    return;
  if (d > ansd)
    return;
  if (R[0] == 0)
  {
    if (ansd > d)
    {
      ansd = d;
      ans_path = path;
    }
    return;
  }
  int c = R[0];
  for (int i = R[0]; i != 0; i = R[i])
  {
    if (S[i] < S[c])
      c = i;
  }
  for (int i = D[c]; i != c; i = D[i])
  {
    remove(i);
    for (int j = R[i]; j != i; j = R[j])
      remove(j);
    path.push_back(Row[i]);
    dance(d + 1, path);
    path.pop_back();
    for (int j = L[i]; j != i; j = L[j])
      resume(j);
    resume(i);
  }
  return;
}

const vector<int> &DLX::get_res() const
{
  return ans_path;
}