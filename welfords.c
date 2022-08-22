#include <R.h>
#include <Rinternals.h>

extern void welfords_wrapper(double *input, int length, int set_size, int *group, int *n, double *mean, double *sd, double *se, double *d, double *t);
  
SEXP welfords(SEXP input, SEXP set_size) {

  int elements = length(input);
  // allocate space for the results
  SEXP group = PROTECT(allocVector(INTSXP, elements));
  SEXP n = PROTECT(allocVector(INTSXP, elements));
  SEXP mean = PROTECT(allocVector(REALSXP, elements));
  SEXP sd = PROTECT(allocVector(REALSXP, elements));
  SEXP se = PROTECT(allocVector(REALSXP, elements));
  SEXP d = PROTECT(allocVector(REALSXP, elements));
  SEXP t = PROTECT(allocVector(REALSXP, elements));

  // run the computation
  welfords_wrapper(REAL(input), elements, asInteger(set_size), INTEGER(group), INTEGER(n), REAL(mean), REAL(sd), REAL(se), REAL(d), REAL(t));


  // allocate space for the 'list'
  SEXP final_result = PROTECT(allocVector(VECSXP, 7));
  // put data into list
  SET_VECTOR_ELT(final_result, 0, group);
  SET_VECTOR_ELT(final_result, 1, n);
  SET_VECTOR_ELT(final_result, 2, mean);
  SET_VECTOR_ELT(final_result, 3, sd);
  SET_VECTOR_ELT(final_result, 4, se);
  SET_VECTOR_ELT(final_result, 5, d);
  SET_VECTOR_ELT(final_result, 6, t);

  // define the attributes of the 'list'
  // first the flass
  SEXP class = PROTECT(mkString("data.frame"));
  setAttrib(final_result, R_ClassSymbol, class);

  // next the row names
  SEXP row_names =   PROTECT(allocVector(INTSXP, 2));
	INTEGER(row_names)[0] = NA_INTEGER;
	INTEGER(row_names)[1] = elements;
	setAttrib(final_result, R_RowNamesSymbol, row_names);

  // next the column names
  SEXP col_names = PROTECT(Rf_allocVector(STRSXP,7));
  SET_STRING_ELT(col_names,0, mkChar("group"));
  SET_STRING_ELT(col_names,1, mkChar("n"));
  SET_STRING_ELT(col_names,2, mkChar("mean"));
  SET_STRING_ELT(col_names,3, mkChar("sd"));
  SET_STRING_ELT(col_names,4, mkChar("se"));
  SET_STRING_ELT(col_names,5, mkChar("d"));
  SET_STRING_ELT(col_names,6, mkChar("t"));
	setAttrib(final_result, R_NamesSymbol, col_names);


  // the 'list' is now a data.frame
  UNPROTECT(11);

  // return the data.frame to R
  return final_result;
}

