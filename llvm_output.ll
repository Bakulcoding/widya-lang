; ModuleID = 'widya_module'
source_filename = "widya_source.wya"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@.str_fmt_num = private unnamed_addr constant [6 x i8] c"%.2f\0A\00", align 1
@.str_fmt_str = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1

declare i32 @printf(i8*, ...)
declare double @sqrt(double)
declare double @pow(double, double)
declare double @sin(double)
declare double @cos(double)
declare double @fabs(double)
declare i32 @puts(i8*)
declare i8* @malloc(i64)
declare void @free(i8*)

; --- External ABI "C" ---
declare double @puts(...)
declare double @abs(...)
declare double @sqrt(...)
declare double @sqlite3_open(...)
declare double @sqlite3_close(...)

define i32 @main() {
entry:
  %t1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double 0.0)
  %t2 = call double @widya_fn_puts(double 0.0)
  %var_nilai_akar = alloca double, align 8
  %t3 = call double @sqrt(double 144.000000)
  store double %t3, double* %var_nilai_akar, align 8
  %t4 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double 0.0)
  %t5 = load double, double* %var_nilai_akar, align 8
  %t6 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double %t5)
  %var_db_status = alloca double, align 8
  %t7 = call double @widya_fn_sqlite3_open(double 0.0, double 0.0)
  store double %t7, double* %var_db_status, align 8
  %t8 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double 0.0)
  %t9 = load double, double* %var_db_status, align 8
  %t10 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double %t9)
  %t11 = call double @widya_fn_sqlite3_close(double 0.0)
  %t12 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double 0.0)
  %t13 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double 0.0)
  %var_GPIO_PORT_A_ADDR = alloca double, align 8
  store double 1073872896.000000, double* %var_GPIO_PORT_A_ADDR, align 8
  %var_REG_DATA_DIR = alloca double, align 8
  store double 1073872900.000000, double* %var_REG_DATA_DIR, align 8
  %t14 = load double, double* %var_GPIO_PORT_A_ADDR, align 8
  %t15 = call double @widya_fn_tulis_volatil(double %t14, double 255.000000)
  %t16 = load double, double* %var_REG_DATA_DIR, align 8
  %t17 = call double @widya_fn_tulis_volatil(double %t16, double 1.000000)
  %var_status_port = alloca double, align 8
  %t18 = load double, double* %var_GPIO_PORT_A_ADDR, align 8
  %t19 = inttoptr i64 (fptosi double %t18 to i64) to double*
  %t20 = load volatile double, double* %t19, align 8
  store double %t20, double* %var_status_port, align 8
  %t21 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double 0.0)
  %t22 = load double, double* %var_status_port, align 8
  %t23 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double %t22)
  %var_buffer = alloca double, align 8
  %t24 = call double @widya_fn_alokasi_buffer_mentah(double 64.000000)
  store double %t24, double* %var_buffer, align 8
  %t25 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double 0.0)
  %t26 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double 0.0)
  %t27 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double 0.0)
  %t28 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double 0.0)
  %t29 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double 0.0)
  ret i32 0
}
