--------------------------------------------------------------------------------
-- Copyright (c) 1995-2013 Xilinx, Inc.  All rights reserved.
--------------------------------------------------------------------------------
--   ____  ____
--  /   /\/   /
-- /___/  \  /    Vendor: Xilinx
-- \   \   \/     Version: P.20131013
--  \   \         Application: netgen
--  /   /         Filename: Equation_Impl_translate.vhd
-- /___/   /\     Timestamp: Tue Nov 02 12:11:09 2021
-- \   \  /  \ 
--  \___\/\___\
--             
-- Command	: -intstyle ise -rpw 100 -tpw 0 -ar Structure -tm Equation_Impl -w -dir netgen/translate -ofmt vhdl -sim Equation_Impl.ngd Equation_Impl_translate.vhd 
-- Device	: 6slx9ftg256-3
-- Input file	: Equation_Impl.ngd
-- Output file	: \\vboxsvr\shared_rw\POCP_Labs\Lab1\netgen\translate\Equation_Impl_translate.vhd
-- # of Entities	: 1
-- Design Name	: Equation_Impl
-- Xilinx	: Y:\xilinx\14.7\ISE_DS\ISE\
--             
-- Purpose:    
--     This VHDL netlist is a verification model and uses simulation 
--     primitives which may not represent the true implementation of the 
--     device, however the netlist is functionally correct and should not 
--     be modified. This file cannot be synthesized and should only be used 
--     with supported simulation tools.
--             
-- Reference:  
--     Command Line Tools User Guide, Chapter 23
--     Synthesis and Simulation Design Guide, Chapter 6
--             
--------------------------------------------------------------------------------

library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
library SIMPRIM;
use SIMPRIM.VCOMPONENTS.ALL;
use SIMPRIM.VPACKAGE.ALL;

entity Equation_Impl is
  port (
    in1 : in STD_LOGIC := 'X'; 
    in2 : in STD_LOGIC := 'X'; 
    in3 : in STD_LOGIC := 'X'; 
    Q : out STD_LOGIC; 
    nQ : out STD_LOGIC 
  );
end Equation_Impl;

architecture Structure of Equation_Impl is
  signal in1_IBUF_0 : STD_LOGIC; 
  signal in2_IBUF_1 : STD_LOGIC; 
  signal in3_IBUF_2 : STD_LOGIC; 
  signal Q_OBUF_3 : STD_LOGIC; 
  signal nQ_OBUF_4 : STD_LOGIC; 
begin
  Q1 : X_LUT3
    generic map(
      INIT => X"E4"
    )
    port map (
      ADR0 => in2_IBUF_1,
      ADR1 => in3_IBUF_2,
      ADR2 => in1_IBUF_0,
      O => Q_OBUF_3
    );
  in1_IBUF : X_BUF
    port map (
      I => in1,
      O => in1_IBUF_0
    );
  in2_IBUF : X_BUF
    port map (
      I => in2,
      O => in2_IBUF_1
    );
  in3_IBUF : X_BUF
    port map (
      I => in3,
      O => in3_IBUF_2
    );
  nQ1 : X_LUT3
    generic map(
      INIT => X"1B"
    )
    port map (
      ADR0 => in2_IBUF_1,
      ADR1 => in3_IBUF_2,
      ADR2 => in1_IBUF_0,
      O => nQ_OBUF_4
    );
  Q_OBUF : X_OBUF
    port map (
      I => Q_OBUF_3,
      O => Q
    );
  nQ_OBUF : X_OBUF
    port map (
      I => nQ_OBUF_4,
      O => nQ
    );
  NlwBlockROC : X_ROC
    generic map (ROC_WIDTH => 100 ns)
    port map (O => GSR);
  NlwBlockTOC : X_TOC
    port map (O => GTS);

end Structure;

