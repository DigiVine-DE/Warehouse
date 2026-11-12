//
// Diese Datei wurde mit der JavaTM Architecture for XML Binding(JAXB) Reference Implementation, v2.2.8-b130911.1802 generiert 
// Siehe <a href="http://java.sun.com/xml/jaxb">http://java.sun.com/xml/jaxb</a> 
// Änderungen an dieser Datei gehen bei einer Neukompilierung des Quellschemas verloren. 
// Generiert: 2021.12.09 um 11:30:16 AM CET 
//


package isoxml.iso11783;

import jakarta.xml.bind.annotation.XmlAccessType;
import jakarta.xml.bind.annotation.XmlAccessorType;
import jakarta.xml.bind.annotation.XmlAttribute;
import jakarta.xml.bind.annotation.XmlRootElement;
import jakarta.xml.bind.annotation.XmlType;
import jakarta.xml.bind.annotation.adapters.HexBinaryAdapter;
import jakarta.xml.bind.annotation.adapters.XmlJavaTypeAdapter;


/**
 * <p>Java-Klasse für anonymous complex type.
 * 
 * <p>Das folgende Schemafragment gibt den erwarteten Content an, der in dieser Klasse enthalten ist.
 * 
 * <pre>
 * &lt;complexType>
 *   &lt;complexContent>
 *     &lt;restriction base="{http://www.w3.org/2001/XMLSchema}anyType">
 *       &lt;attribute name="A" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}unsignedShort">
 *             &lt;minInclusive value="1"/>
 *             &lt;maxInclusive value="65534"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="B" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}hexBinary">
 *             &lt;length value="2"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="C" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}unsignedByte">
 *             &lt;minInclusive value="0"/>
 *             &lt;maxInclusive value="7"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="D" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}unsignedByte">
 *             &lt;minInclusive value="0"/>
 *             &lt;maxInclusive value="31"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="E">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}string">
 *             &lt;maxLength value="32"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="F">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}unsignedShort">
 *             &lt;minInclusive value="1"/>
 *             &lt;maxInclusive value="65534"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *     &lt;/restriction>
 *   &lt;/complexContent>
 * &lt;/complexType>
 * </pre>
 * 
 * 
 */
@XmlAccessorType(XmlAccessType.FIELD)
@XmlType(name = "")
@XmlRootElement(name = "DPD")
public class DPD {

    @XmlAttribute(name = "A", required = true)
    protected int a;
    @XmlAttribute(name = "B", required = true)
    @XmlJavaTypeAdapter(HexBinaryAdapter.class)
    protected byte[] b;
    @XmlAttribute(name = "C", required = true)
    protected short c;
    @XmlAttribute(name = "D", required = true)
    protected short d;
    @XmlAttribute(name = "E")
    protected String e;
    @XmlAttribute(name = "F")
    protected Integer f;

    /**
     * Ruft den Wert der a-Eigenschaft ab.
     * 
     */
    public int getA() {
        return a;
    }

    /**
     * Legt den Wert der a-Eigenschaft fest.
     * 
     */
    public void setA(int value) {
        this.a = value;
    }

    /**
     * Ruft den Wert der b-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public byte[] getB() {
        return b;
    }

    /**
     * Legt den Wert der b-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setB(byte[] value) {
        this.b = value;
    }

    /**
     * Ruft den Wert der c-Eigenschaft ab.
     * 
     */
    public short getC() {
        return c;
    }

    /**
     * Legt den Wert der c-Eigenschaft fest.
     * 
     */
    public void setC(short value) {
        this.c = value;
    }

    /**
     * Ruft den Wert der d-Eigenschaft ab.
     * 
     */
    public short getD() {
        return d;
    }

    /**
     * Legt den Wert der d-Eigenschaft fest.
     * 
     */
    public void setD(short value) {
        this.d = value;
    }

    /**
     * Ruft den Wert der e-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public String getE() {
        return e;
    }

    /**
     * Legt den Wert der e-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setE(String value) {
        this.e = value;
    }

    /**
     * Ruft den Wert der f-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link Integer }
     *     
     */
    public Integer getF() {
        return f;
    }

    /**
     * Legt den Wert der f-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link Integer }
     *     
     */
    public void setF(Integer value) {
        this.f = value;
    }

}
