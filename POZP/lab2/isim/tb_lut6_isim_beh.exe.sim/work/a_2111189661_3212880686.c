/**********************************************************************/
/*   ____  ____                                                       */
/*  /   /\/   /                                                       */
/* /___/  \  /                                                        */
/* \   \   \/                                                       */
/*  \   \        Copyright (c) 2003-2009 Xilinx, Inc.                */
/*  /   /          All Right Reserved.                                 */
/* /---/   /\                                                         */
/* \   \  /  \                                                      */
/*  \___\/\___\                                                    */
/***********************************************************************/

/* This file is designed for use with ISim build 0x7708f090 */

#define XSI_HIDE_SYMBOL_SPEC true
#include "xsi.h"
#include <memory.h>
#ifdef __GNUC__
#include <stdlib.h>
#else
#include <malloc.h>
#define alloca _alloca
#endif
static const char *ng0 = "C:/xil/lab2/LUT6_beh.vhd";
extern char *IEEE_P_3620187407;

int ieee_p_3620187407_sub_514432868_3965413181(char *, char *, char *);


static void work_a_2111189661_3212880686_p_0(char *t0)
{
    char *t1;
    char *t3;
    char *t4;
    int t5;
    int t6;
    unsigned int t7;
    unsigned int t8;
    unsigned int t9;
    char *t10;
    unsigned char t11;
    char *t12;
    char *t13;
    char *t14;
    char *t15;
    char *t16;
    char *t17;

LAB0:    xsi_set_current_line(44, ng0);

LAB3:    t1 = (t0 + 4438);
    t3 = (t0 + 1032U);
    t4 = *((char **)t3);
    t3 = (t0 + 4344U);
    t5 = ieee_p_3620187407_sub_514432868_3965413181(IEEE_P_3620187407, t4, t3);
    t6 = (t5 - 63);
    t7 = (t6 * -1);
    xsi_vhdl_check_range_of_index(63, 0, -1, t5);
    t8 = (1U * t7);
    t9 = (0 + t8);
    t10 = (t1 + t9);
    t11 = *((unsigned char *)t10);
    t12 = (t0 + 2872);
    t13 = (t12 + 56U);
    t14 = *((char **)t13);
    t15 = (t14 + 56U);
    t16 = *((char **)t15);
    *((unsigned char *)t16) = t11;
    xsi_driver_first_trans_fast_port(t12);

LAB2:    t17 = (t0 + 2792);
    *((int *)t17) = 1;

LAB1:    return;
LAB4:    goto LAB2;

}


extern void work_a_2111189661_3212880686_init()
{
	static char *pe[] = {(void *)work_a_2111189661_3212880686_p_0};
	xsi_register_didat("work_a_2111189661_3212880686", "isim/tb_lut6_isim_beh.exe.sim/work/a_2111189661_3212880686.didat");
	xsi_register_executes(pe);
}
