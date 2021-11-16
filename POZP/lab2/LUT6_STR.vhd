----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    18:25:48 11/16/2021 
-- Design Name: 
-- Module Name:    LUT6_STR - Structural 
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

entity LUT6_STR is
    Port ( addr : in  STD_LOGIC_VECTOR (5 downto 0);
           Q : out  STD_LOGIC);
end LUT6_STR;

architecture Behavioral of LUT6_STR is

component LUT_5 is
	GENERIC (
		INIT: STD_LOGIC_VECTOR (31 downto 0) := x"000000FF"
	);
   Port ( addr : in  STD_LOGIC_VECTOR (4 downto 0);
          Q : out  STD_LOGIC);
end component;

signal tmp, tmp2: STD_LOGIC;

begin

u1: LUT_5 generic map (x"FF0000FF")
	port map (addr(4 downto 0), tmp);
	
u2: LUT_5 generic map (x"FF0000FF")
	port map (addr(4 downto 0), tmp2);
	
Q <= tmp when addr(5) = '0' else tmp2;

end Behavioral;


