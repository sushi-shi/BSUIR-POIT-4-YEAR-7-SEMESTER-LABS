----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    23:02:11 01/11/2021 
-- Design Name: 
-- Module Name:    Equation_Impl - Behavioral 
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

entity Equation_Impl is
    Port ( in1 : in STD_LOGIC;
	        in2 : in STD_LOGIC;
			  in3 : in STD_LOGIC;
			  Q : out STD_LOGIC;
			  nQ : out STD_LOGIC);
end Equation_Impl;

architecture Behavioral of Equation_Impl is
signal tempQ : STD_LOGIC;
begin
    tempQ <= (in1 and in2) or (in3 and not in2);
	 nQ <= not tempQ;
	 Q <= tempQ;
end Behavioral;

