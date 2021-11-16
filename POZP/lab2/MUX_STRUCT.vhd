----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    13:35:57 11/16/2021 
-- Design Name: 
-- Module Name:    MUX_STRUCT - Behavioral 
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

entity MUX_STRUCT is
    Port ( A : in  STD_LOGIC;
           B : in  STD_LOGIC;
           S : in  STD_LOGIC;
           Z : out  STD_LOGIC);
end MUX_STRUCT;

architecture Structural of MUX_STRUCT is
component INV
	port (A : in STD_LOGIC;
			S : out STD_LOGIC);
end component;
component AND2
    port ( A : in  STD_LOGIC;
           B : in  STD_LOGIC;
           S : out  STD_LOGIC);
end component;
component OR2
    port ( A : in  STD_LOGIC;
           B : in  STD_LOGIC;
           S : out  STD_LOGIC);
end component;

signal S_N, tmp2, tmp3 : STD_LOGIC;

begin

	U1: INV port map (A => S, S => S_N);
	U2: AND2 port map (A => A, B => S_N, S => tmp2);
	U3: AND2 port map (A => S, B => B, S => tmp3);
	U4: OR2 port map (A => tmp2, B => tmp3, S => Z);
	

end Structural;

