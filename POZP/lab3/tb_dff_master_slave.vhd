----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    00:06:53 11/20/2021 
-- Design Name: 
-- Module Name:    tb_dff_master_slave - Behavioral 
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

entity tb_dff_master_slave is
end tb_dff_master_slave;

architecture Behavioral of tb_dff_master_slave is
    COMPONENT DFF_MASTER_SLAVE_V2
    PORT(
         D : IN  std_logic;
			CLK : IN std_logic;
         Q : OUT  std_logic
        );
    END COMPONENT;
    
   --Inputs
   signal D : std_logic := '0';

	
   signal Q : std_logic;
   -- No clocks detected in port list. Replace <clock> below with 
   -- appropriate port name 
   signal CLK : std_logic := '0';


   -- Clock period definitions
   constant Clk_period : time := 100 ns;
BEGIN
 
	-- Instantiate the Unit Under Test (UUT)
   uut1: DFF_MASTER_SLAVE_V2 PORT MAP (
          D => D,
          CLK => CLK,
          Q => Q
        );
		  
	Clk_process :process
   begin
		CLK <= '0';
		wait for Clk_period/2;
		CLK <= '1';
		wait for Clk_period/2;
   end process;

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
	

END;
