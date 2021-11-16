----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    14:31:09 11/16/2021 
-- Design Name: 
-- Module Name:    DEVICE_1_STRUCT - Behavioral 
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

entity DEVICE_1_STRUCT is
    Port ( X : in  STD_LOGIC;
           Y : in  STD_LOGIC;
           Z : in  STD_LOGIC;
           S : out  STD_LOGIC);
end DEVICE_1_STRUCT;

architecture Structural of DEVICE_1_STRUCT is
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
component AND3
    port ( A : in  STD_LOGIC;
           B : in  STD_LOGIC;
			  C : in  STD_LOGIC;
           S : out  STD_LOGIC);
end component;

signal X_N, Y_N, Z_N: STD_LOGIC;
signal tmp1, tmp2, tmp3 : STD_LOGIC;

begin
	U1: INV port map (A => X, S => X_N);
	U2: INV port map (A => Y, S => Y_N);
	U3: INV port map (A => Z, S => Z_N);
	
	U4: OR2 port map (A => X, B => Y_N, S => tmp1);
	U5: AND2 port map (A => tmp1, B => Z, S => tmp2);
	U6: AND3 port map (A => X_N, B => Y, C => Z_N, S => tmp3);
	U7: OR2 port map (A => tmp2, B => tmp3, S => S);


end Structural;

