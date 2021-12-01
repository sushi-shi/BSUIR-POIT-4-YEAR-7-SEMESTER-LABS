----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    21:21:07 11/20/2021 
-- Design Name: 
-- Module Name:    tb_signature_analyzer - Behavioral 
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

entity tb_signature_analyzer is
end tb_signature_analyzer;

architecture Behavioral of tb_signature_analyzer is
	-- q(x) = x + 1      | factor    | Q_VEC_OUT
	-- s(x) = x ^ 2 + x  | remainder | Pout
    COMPONENT SIGNATURE_ANALYZER
    port (
		CLK : in std_logic;
		RST : in std_logic;
		Pin : in std_logic;
		Pout: out std_logic_vector(0 to 5);
		Qout: out std_logic
	);
	end COMPONENT;
	 
   --Inputs
   signal CLK : std_logic := '0';
	signal RST : std_logic := '0';
	
	
	signal P_VEC : std_logic_vector(0 to 7) := "11000011";
	signal Pin : std_logic := '0';
	
	signal STARTED : std_logic := '0';

 	--Outputs
   signal Pout : std_logic_vector(0 to 5);
	signal Qout : std_logic;
	
	signal Q_VEC_OUT : std_logic_vector(0 to 7) := "00000000";

   constant Clk_period : time := 50 ns;
	
BEGIN
 
	-- Instantiate the Unit Under Test (UUT)
   uut1: SIGNATURE_ANALYZER PORT MAP (
			 CLK => CLK,
			 RST => RST,
			 Pin => Pin,
			 Pout => Pout,
			 Qout => Qout
        );

 
   Clk_process: process
   begin
		CLK <= not(CLK);
		wait for Clk_period/2;
   end process;
	
	react_proc: process(CLK, STARTED)
	begin
		if STARTED = '1' then
			Pin <= P_VEC(0);
			
			if CLK = '0' then
				P_VEC <= P_VEC(1 to 7) & '0';
				Q_VEC_OUT   <= Qout & Q_VEC_OUT(0 to 6);
			end if;
		end if;
			
	end process;
	
   stim_proc: process
   begin		
		wait for CLK_period;
		
		RST <= '1'; wait for 2 * CLK_period;
		RST <= '0'; wait for CLK_period;
		STARTED <= '1';
				
      wait;
   end process;
END;