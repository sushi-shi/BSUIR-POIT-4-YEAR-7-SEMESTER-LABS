----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    17:50:06 11/19/2021 
-- Design Name: 
-- Module Name:    tb_tff - Behavioral 
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

entity tb_tff is
end tb_tff;

ARCHITECTURE behavior OF tb_tff IS 
 
    -- Component Declaration for the Unit Under Test (UUT)
 
    COMPONENT TFF
    PORT(
         T : IN  std_logic;
         CLK : IN  std_logic;
         CLR : IN  std_logic;
         Q : OUT  std_logic);
    END COMPONENT;
    

   --Inputs
   signal T : std_logic := '0';
   signal CLK : std_logic := '0';
   signal CLR : std_logic := '0';

 	--Outputs
   signal Q : std_logic;

   -- Clock period definitions
   constant Clk_period : time := 10 ns;
 
BEGIN
 
	-- Instantiate the Unit Under Test (UUT)
   uut: TFF PORT MAP (
          T => T,
          CLK => CLK,
          CLR => CLR,
          Q => Q
        );

   -- Clock process definitions
   Clk_process :process
   begin
		CLK <= '0';
		wait for Clk_period/2;
		CLK <= '1';
		wait for Clk_period/2;
   end process;
 

   -- Stimulus process
   stim_proc: process
   begin		
      -- hold reset state for 100 ns.
      CLR <= '1';
      wait for 100 ns;	
      CLR <= '0';

      wait for Clk_period;
      T <= '1';
      -- insert stimulus here 

      wait;
   end process;

END;
