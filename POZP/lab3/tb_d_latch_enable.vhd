----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    23:43:44 11/19/2021 
-- Design Name: 
-- Module Name:    tb_d_latch_enable - Behavioral 
-- Project Name: 
-- Target Devices: 
-- Tool versions: 
-- Description: 
--
-- Dependencies: 
--
-- Revision: 
-- Revision 0.01 - File Created
-- Additional Comments: 
--
----------------------------------------------------------------------------------
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
--use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx primitives in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity tb_d_latch_enable is
end tb_d_latch_enable;

architecture Behavioral of tb_d_latch_enable is
 COMPONENT D_LATCH_ENABLE_BHVR
    PORT(
         D : IN  std_logic;
         E : IN  std_logic;
         Q : INOUT  std_logic;
         nQ : INOUT  std_logic
        );
    END COMPONENT;
    
	 COMPONENT D_LATCH_ENABLE
    PORT(
         D : IN  std_logic;
         E : IN  std_logic;
         Q : INOUT  std_logic;
         nQ : INOUT  std_logic
        );
    END COMPONENT;
	 
	signal D : std_logic := '0';
   signal E : std_logic := '0';

   signal Q_BHVR : std_logic;
   signal nQ_BHVR : std_logic;
	signal Q : std_logic;
   signal nQ: std_logic;
	
	signal error: std_logic;
begin
   uut1: D_LATCH_ENABLE_BHVR PORT MAP (
          D => D,
          E => E,
          Q => Q_BHVR,
          nQ => nQ_BHVR
        );
   uut2: D_LATCH_ENABLE PORT MAP (
          D => D,
          E => E,
          Q => Q,
          nQ => nQ
        );
		  
   stim_proc: process
   begin		
      -- hold reset state for 100 ns.
		wait for 100 ns;	
		D <= '0';
		E <= '1';
		
		wait for 100 ns;	
      D <= '1';
		E <= '0';
		
		wait for 100 ns;	
      D <= '1';
		E <= '1';
		
		wait for 100 ns;	
      D <= '0';
		E <= '1';
		
		wait for 100 ns;	
      D <= '1';
		E <= '0';
   end process;
	
	error <= (Q XOR Q_BHVR) OR (nQ_BHVR XOR nQ);
end;
