----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    21:06:58 11/20/2021 
-- Design Name: 
-- Module Name:    SIGNATURE_ANALYZER - Behavioral 
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

entity SIGNATURE_ANALYZER is
    generic (
	 -- f(x) = 1 xor x xor x^6;
		alpha : std_logic_vector := "1100001"
	 );
    port (
		CLK : in std_logic;
		RST : in std_logic;
		Pin : in std_logic;
		Pout: out std_logic_vector(0 to alpha'high - 1);
		Qout: out std_logic
	);
end SIGNATURE_ANALYZER;

architecture Behavioral of SIGNATURE_ANALYZER is

signal sreg: std_logic_vector(0 to alpha'high - 1);
signal qreg: std_logic;

signal sdat: std_logic_vector(0 to alpha'high - 1);
signal qdat: std_logic;

begin
  main: process( CLK, RST, sdat, qdat )
  begin
    if RST = '1' then
      sreg <= ( others => '0' );
		qreg <= '0';
    elsif rising_edge(CLK) then
      sreg <= sdat;
		qreg <= qdat;
    end if;
  end process;
  
  data: process( sreg, qreg, Pin ) 
    variable zerostate : std_logic;
  begin	 
	 for i in 0 to alpha'high - 2 loop
		zerostate:= zerostate or sreg(i);
		
		if alpha(i+1) = '1' then
			sdat(i+1) <= sreg( alpha'high - 1 ) xor sreg(i);
		else 
			sdat(i+1) <= sreg( i );
		end if;
	 end loop;

	 sdat(0) <= sreg( alpha'high - 1 ) xor Pin;
	 qdat <= sreg( alpha'high - 1 );

  end process;

  Pout <= sdat;
  Qout <= qdat;
    
end Behavioral;