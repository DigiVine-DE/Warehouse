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
import jakarta.xml.bind.annotation.XmlElement;
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
 *       &lt;choice minOccurs="0">
 *         &lt;element ref="{}ASP" minOccurs="0"/>
 *       &lt;/choice>
 *       &lt;attribute name="A" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}hexBinary">
 *             &lt;length value="8"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="B" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}hexBinary">
 *             &lt;length value="8"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="C" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}hexBinary">
 *             &lt;minLength value="7"/>
 *             &lt;maxLength value="39"/>
 *             &lt;pattern value="((([0-9]|[a-e]|[A-E])([0-9]|[a-f]|[A-F]))|((F|f)([0-9]|[a-e]|[A-E]))){7}(([0-9]|[a-f]|[A-F])([0-9]|[a-f]|[A-F]))*"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="D" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}hexBinary">
 *             &lt;minLength value="7"/>
 *             &lt;maxLength value="39"/>
 *             &lt;pattern value="((([0-9]|[A-E]|[a-e])([0-9]|[A-F]|[a-f]))|((F|f)([0-9]|[A-E]|[a-e]))){7}(([0-9]|[A-F]|[a-f])([0-9]|[A-F]|[a-f]))*"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="E" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}unsignedShort">
 *             &lt;minInclusive value="0"/>
 *             &lt;maxInclusive value="4095"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="F" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}unsignedShort">
 *             &lt;minInclusive value="0"/>
 *             &lt;maxInclusive value="4095"/>
 *           &lt;/restriction>
 *         &lt;/simpleType>
 *       &lt;/attribute>
 *       &lt;attribute name="G" use="required">
 *         &lt;simpleType>
 *           &lt;restriction base="{http://www.w3.org/2001/XMLSchema}hexBinary">
 *             &lt;length value="2"/>
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
@XmlType(name = "", propOrder = {
    "asp"
})
@XmlRootElement(name = "CAT")
public class CAT {

    @XmlElement(name = "ASP")
    protected ASP asp;
    @XmlAttribute(name = "A", required = true)
    @XmlJavaTypeAdapter(HexBinaryAdapter.class)
    protected byte[] a;
    @XmlAttribute(name = "B", required = true)
    @XmlJavaTypeAdapter(HexBinaryAdapter.class)
    protected byte[] b;
    @XmlAttribute(name = "C", required = true)
    @XmlJavaTypeAdapter(HexBinaryAdapter.class)
    protected byte[] c;
    @XmlAttribute(name = "D", required = true)
    @XmlJavaTypeAdapter(HexBinaryAdapter.class)
    protected byte[] d;
    @XmlAttribute(name = "E", required = true)
    protected int e;
    @XmlAttribute(name = "F", required = true)
    protected int f;
    @XmlAttribute(name = "G", required = true)
    @XmlJavaTypeAdapter(HexBinaryAdapter.class)
    protected byte[] g;

    /**
     * 
     * 							AllocationStamp
     * 						
     * 
     * @return
     *     possible object is
     *     {@link ASP }
     *     
     */
    public ASP getASP() {
        return asp;
    }

    /**
     * Legt den Wert der asp-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link ASP }
     *     
     */
    public void setASP(ASP value) {
        this.asp = value;
    }

    /**
     * Ruft den Wert der a-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public byte[] getA() {
        return a;
    }

    /**
     * Legt den Wert der a-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setA(byte[] value) {
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
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public byte[] getC() {
        return c;
    }

    /**
     * Legt den Wert der c-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setC(byte[] value) {
        this.c = value;
    }

    /**
     * Ruft den Wert der d-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public byte[] getD() {
        return d;
    }

    /**
     * Legt den Wert der d-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setD(byte[] value) {
        this.d = value;
    }

    /**
     * Ruft den Wert der e-Eigenschaft ab.
     * 
     */
    public int getE() {
        return e;
    }

    /**
     * Legt den Wert der e-Eigenschaft fest.
     * 
     */
    public void setE(int value) {
        this.e = value;
    }

    /**
     * Ruft den Wert der f-Eigenschaft ab.
     * 
     */
    public int getF() {
        return f;
    }

    /**
     * Legt den Wert der f-Eigenschaft fest.
     * 
     */
    public void setF(int value) {
        this.f = value;
    }

    /**
     * Ruft den Wert der g-Eigenschaft ab.
     * 
     * @return
     *     possible object is
     *     {@link String }
     *     
     */
    public byte[] getG() {
        return g;
    }

    /**
     * Legt den Wert der g-Eigenschaft fest.
     * 
     * @param value
     *     allowed object is
     *     {@link String }
     *     
     */
    public void setG(byte[] value) {
        this.g = value;
    }

}
