----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    23:02:21 11/19/2021 
-- Design Name: 
-- Module Name:    tb_rs_latch - Behavioral 
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

entity tb_rs_latch is
end tb_rs_latch;

architecture Behavioral of tb_rs_latch is
 COMPONENT RS_LATCH_BHVR
    PORT(
         S : IN  std_logic;
         R : IN  std_logic;
         Q : INOUT  std_logic;
         nQ : INOUT  std_logic
        );
    END COMPONENT;
    
	 COMPONENT RS_LATCH
    PORT(
         S : IN  std_logic;
         R : IN  std_logic;
         Q : INOUT  std_logic;
         nQ : INOUT  std_logic
        );
    END COMPONENT;
	 
	signal S : std_logic := '0';
   signal R : std_logic := '0';

   signal Q_BHVR : std_logic;
   signal nQ_BHVR : std_logic;
	signal Q : std_logic;
   signal nQ: std_logic;
	
	signal error: std_logic;
begin
   uut1: RS_LATCH_BHVR PORT MAP (
          S => S,
          R => R,
          Q => Q_BHVR,
          nQ => nQ_BHVR
        );
   uut2: RS_LATCH PORT MAP (
          S => S,
          R => R,
          Q => Q,
          nQ => nQ
        );
		  
   stim_proc: process
   begin		
      -- hold reset state for 100 ns.
      wait for 100 ns;	

		S <= '0';
		R <= '1';
		
		wait for 100 ns;	

      S <= '1';
		R <= '0';
	
		wait for 100 ns;	

      S <= '1';
		R <= '1';
		
		wait for 100 ns;	
		
      S <= '0';
		R <= '0';
		
		wait for 100 ns;	

      S <= '1';
		R <= '1';
      -- insert stimulus here 

      wait;
   end process;
	
	error <= (Q XOR Q_BHVR) OR (nQ_BHVR XOR nQ);
end;

