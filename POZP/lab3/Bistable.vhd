----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    18:24:20 11/19/2021 
-- Design Name: 
-- Module Name:    Bistable - Behavioral 
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

entity Bistable is
    Port ( Q : out  STD_LOGIC;
           nQ : out  STD_LOGIC);
end Bistable;

architecture Structural of Bistable is


component MY_NOT port (
	A: in STD_LOGIC;
	Q: out STD_LOGIC
);
end component;

signal x : STD_LOGIC:= '0';
signal nx: STD_LOGIC;

begin
	U1: MY_NOT port map (A=>x, Q=>nx);
	U2: MY_NOT port map (A=>nx, Q=>x);
	Q <= x;
	nQ <= nx;
end Structural;

