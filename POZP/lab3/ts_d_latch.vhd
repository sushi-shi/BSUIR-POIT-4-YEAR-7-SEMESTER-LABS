----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    23:26:59 11/19/2021 
-- Design Name: 
-- Module Name:    ts_d_latch - Behavioral 
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

entity ts_d_latch is
end ts_d_latch;

architecture Behavioral of ts_d_latch is
    COMPONENT D_LATCH
    PORT(
         D : IN  std_logic;
         Q : INOUT  std_logic;
         nQ : INOUT  std_logic
        );
    END COMPONENT;
    
    COMPONENT D_LATCH_BHVR
    PORT(
         D : IN  std_logic;
         Q : INOUT  std_logic;
         nQ : INOUT  std_logic
        );
    END COMPONENT;
   --Inputs
   signal D : std_logic := '0';

 	--Outputs
	
   signal Q_BHVR : std_logic;
   signal nQ_BHVR : std_logic;
   signal Q : std_logic;
   signal nQ : std_logic;
   -- No clocks detected in port list. Replace <clock> below with 
   -- appropriate port name 
	
	signal error: std_logic; 
BEGIN
 
	-- Instantiate the Unit Under Test (UUT)
   uut1: D_LATCH PORT MAP (
          D => D,
          Q => Q,
          nQ => nQ
        );
		  
   uut2: D_LATCH_BHVR PORT MAP (
          D => D,
          Q => Q_BHVR,
          nQ => nQ_BHVR
        );

   stim_proc: process
   begin		
      -- hold reset state for 100 ns.
      wait for 100 ns;	
		D <= '1';
		
		wait for 100 ns;	
      D <= '0';
		
      wait for 100 ns;	
		D <= '1';
		
		wait for 100 ns;	
      D <= '0';
      -- insert stimulus here 

      wait;
   end process;
	error <= (Q XOR Q_BHVR) OR (nQ_BHVR XOR nQ);

END;
