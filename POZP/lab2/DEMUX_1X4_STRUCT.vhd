----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    15:33:05 11/16/2021 
-- Design Name: 
-- Module Name:    DEMUX_1X4_STRUCT - Behavioral 
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

entity DEMUX_1X4_STRUCT is
    Port ( X : in  STD_LOGIC;
           S : in  STD_LOGIC_VECTOR (1 downto 0);
           Z : out  STD_LOGIC_VECTOR (3 downto 0));
end DEMUX_1X4_STRUCT;

architecture Structural of DEMUX_1X4_STRUCT is
component INV
	port (A : in STD_LOGIC;
			S : out STD_LOGIC);
end component;
component AND3
    port ( A : in  STD_LOGIC;
           B : in  STD_LOGIC;
           C : in  STD_LOGIC;
           S : out  STD_LOGIC);
end component;
signal N_S : STD_LOGIC_VECTOR(1 downto 0);
begin
	U1: INV port map (A => S(0), S => N_S(0));
	U2: INV port map (A => S(1), S => N_S(1));
	
	U3: AND3 port map (A => X, B => N_S(0), C => N_S(1), S => Z(0));
	U4: AND3 port map (A => X, B => N_S(1), C => S(0), S => Z(1));
	U5: AND3 port map (A => X, B => N_S(0), C => S(1), S => Z(2));
	U6: AND3 port map (A => X, B => S(0), C=> S(1), S => Z(3));

end Structural;

