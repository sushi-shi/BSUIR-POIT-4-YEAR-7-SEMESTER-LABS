----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    22:59:05 01/11/2021 
-- Design Name: 
-- Module Name:    AND4_Impl - Behavioral 
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

entity AND4_Impl is
    Port ( A : in STD_LOGIC;
	        B : in STD_LOGIC;
			  C : in STD_LOGIC;
			  D : in STD_LOGIC;
			  Q : out STD_LOGIC);
end AND4_Impl;

architecture Behavioral of AND4_Impl is

begin
	Q <= A and B and C and D;
end Behavioral;

