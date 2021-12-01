----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    22:42:46 11/20/2021 
-- Design Name: 
-- Module Name:    tb_lfsr_into_signature_analyzer - Behavioral 
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

entity tb_lfsr_into_signature_analyzer is
end tb_lfsr_into_signature_analyzer;


architecture Behavioral of tb_lfsr_into_signature_analyzer is
    COMPONENT SIGNATURE_ANALYZER
    port (
		CLK : in std_logic;
		RST : in std_logic;
		Pin : in std_logic;
		Pout: out std_logic_vector(0 to 5);
		Qout: out std_logic
	);
	end COMPONENT;
	
	 COMPONENT LFSR_INNER
    port (
		CLK : in std_logic;
		RST : in std_logic;
		Pout: out std_logic_vector(0 to 5)
	 );
	 END COMPONENT;
	  
   --Inputs
   signal CLK : std_logic := '0';
	signal NXT : std_logic := '0';
	signal RST : std_logic := '0';
		

 	--Outputs
   signal Pout : std_logic_vector(0 to 5);
	signal Fout : std_logic_vector(0 to 5);
	
   constant Clk_period : time := 50 ns;
	
BEGIN
 
	-- Instantiate the Unit Under Test (UUT)
   uut1: SIGNATURE_ANALYZER PORT MAP (
			 CLK => CLK,
			 RST => RST,
			 Pin => Fout(5),
			 Pout => Pout,
			 Qout => open
        );
		  
   uut2: LFSR_INNER PORT MAP (
			 CLK => CLK,
			 RST => RST,
			 Pout => Fout
        );
 
   Clk_process: process
   begin
		CLK <= not(CLK);
		wait for Clk_period/2;
   end process;
	
	
   stim_proc: process
   begin		
		wait for CLK_period;
		
		RST <= '1'; wait for 2 * CLK_period;
		RST <= '0'; wait for CLK_period;
				
      wait;
   end process;
END;
